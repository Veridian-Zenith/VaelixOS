//! Hybrid CPU Scheduler
//!
//! Implements task scheduling for hybrid CPU architectures:
//! - P-core vs E-core task placement
//! - Workload characterization
//! - Power-aware scheduling
//! - Thread migration policies

use crate::HalError;
use crate::power::policy::{PolicyManager, PolicyMode};
use crate::drivers::cpu_hybrid::{HybridCpuDriver, CoreType};
use core::sync::atomic::{AtomicU32, AtomicBool, Ordering};
use alloc::collections::{BTreeMap, VecDeque};
use alloc::vec::Vec;

/// Task characteristics for scheduling
///
/// This struct represents the characteristics of a task for scheduling purposes.
#[derive(Debug, Clone)]
pub struct TaskProfile {
    /// Unique task ID
    task_id: u32,
    /// Task priority
    priority: u8,
    /// CPU utilization (0-1)
    cpu_intensity: f32,
    /// Memory access rate (0-1)
    memory_intensity: f32,
    /// I/O operation rate (0-1)
    io_intensity: f32,
    /// Last core the task ran on
    last_core: u32,
    /// Total runtime in milliseconds
    run_time: u64,
    /// Optional deadline in milliseconds
    deadline: Option<u64>,
}

/// Core load information
///
/// This struct represents the load information for a core.
#[derive(Debug)]
struct CoreLoad {
    /// Core ID
    core_id: u32,
    /// Core type (Performance or Efficiency)
    core_type: CoreType,
    /// Core utilization (0-1)
    utilization: f32,
    /// Number of tasks on the core
    task_count: u32,
    /// Active tasks on the core
    active_tasks: VecDeque<TaskProfile>,
}

/// Scheduler configuration
///
/// This struct represents the configuration for the hybrid scheduler.
#[derive(Debug, Clone)]
pub struct SchedulerConfig {
    /// Load imbalance threshold to trigger migration (0-1)
    migration_threshold: f32,
    /// Bias for P-core assignment (0-1)
    p_core_preference: f32,
    /// Power efficiency weight (0-1)
    power_efficiency: f32,
}

/// Hybrid scheduler state
///
/// This struct represents the state of the hybrid scheduler.
#[derive(Debug)]
pub struct HybridScheduler {
    /// Initialized flag
    initialized: AtomicBool,
    /// Total number of tasks
    total_tasks: AtomicU32,
    /// Core loads
    core_loads: Vec<CoreLoad>,
    /// Task profiles
    task_profiles: BTreeMap<u32, TaskProfile>,
    /// Scheduler configuration
    config: SchedulerConfig,
}

// Singleton scheduler
static mut HYBRID_SCHEDULER: Option<HybridScheduler> = None;

impl HybridScheduler {
    /// Initialize hybrid scheduler
    ///
    /// This function initializes the hybrid scheduler. It sets up the core loads and prepares the scheduler for operation.
    pub fn init() -> Result<(), HalError> {
        unsafe {
            if HYBRID_SCHEDULER.is_some() {
                return Ok(());
            }

            // Get CPU topology
            let cpu_driver = HybridCpuDriver::driver();
            let topology = cpu_driver.get_topology();

            // Initialize core loads
            let mut core_loads = Vec::new();
            for core in &topology {
                core_loads.push(CoreLoad {
                    core_id: core.core_id,
                    core_type: core.core_type,
                    utilization: 0.0,
                    task_count: 0,
                    active_tasks: VecDeque::new(),
                });
            }

            HYBRID_SCHEDULER = Some(HybridScheduler {
                initialized: AtomicBool::new(true),
                total_tasks: AtomicU32::new(0),
                core_loads,
                task_profiles: BTreeMap::new(),
                config: SchedulerConfig {
                    migration_threshold: 0.2,
                    p_core_preference: 0.7,
                    power_efficiency: 0.5,
                },
            });

            Ok(())
        }
    }

    /// Schedule a new task
    ///
    /// This function schedules a new task. It selects the target core and assigns the task to it.
    ///
    /// # Arguments
    ///
    /// * `profile` - The task profile to schedule.
    ///
    /// # Returns
    ///
    /// * `Result<u32, HalError>` - A result containing the target core ID or an error.
    pub fn schedule_task(profile: TaskProfile) -> Result<u32, HalError> {
        unsafe {
            let scheduler = HYBRID_SCHEDULER.as_mut().ok_or(HalError::NotInitialized)?;
            if !scheduler.initialized.load(Ordering::SeqCst) {
                return Err(HalError::NotInitialized);
            }

            // Select target core
            let target_core = select_target_core(scheduler, &profile)?;

            // Assign task to core
            if let Some(core_load) = scheduler.core_loads.iter_mut()
                .find(|c| c.core_id == target_core) {
                core_load.active_tasks.push_back(profile.clone());
                core_load.task_count += 1;
                scheduler.task_profiles.insert(profile.task_id, profile);
                scheduler.total_tasks.fetch_add(1, Ordering::SeqCst);
            }

            Ok(target_core)
        }
    }

    /// Update task profile
    ///
    /// This function updates the profile of an existing task. It also evaluates if the task should migrate to another core.
    ///
    /// # Arguments
    ///
    /// * `task_id` - The ID of the task to update.
    /// * `profile` - The new task profile.
    ///
    /// # Returns
    ///
    /// * `Result<(), HalError>` - A result indicating success or an error.
    pub fn update_task(task_id: u32, profile: TaskProfile) -> Result<(), HalError> {
        unsafe {
            let scheduler = HYBRID_SCHEDULER.as_mut().ok_or(HalError::NotInitialized)?;
            if !scheduler.initialized.load(Ordering::SeqCst) {
                return Err(HalError::NotInitialized);
            }

            // Update profile
            scheduler.task_profiles.insert(task_id, profile.clone());

            // Check if task should migrate
            evaluate_task_migration(scheduler, &profile)?;

            Ok(())
        }
    }

    /// Remove completed task
    ///
    /// This function removes a completed task from the scheduler.
    ///
    /// # Arguments
    ///
    /// * `task_id` - The ID of the task to remove.
    ///
    /// # Returns
    ///
    /// * `Result<(), HalError>` - A result indicating success or an error.
    pub fn complete_task(task_id: u32) -> Result<(), HalError> {
        unsafe {
            let scheduler = HYBRID_SCHEDULER.as_mut().ok_or(HalError::NotInitialized)?;
            if !scheduler.initialized.load(Ordering::SeqCst) {
                return Err(HalError::NotInitialized);
            }

            // Find and remove task
            if let Some(profile) = scheduler.task_profiles.remove(&task_id) {
                if let Some(core_load) = scheduler.core_loads.iter_mut()
                    .find(|c| c.core_id == profile.last_core) {
                    core_load.active_tasks.retain(|t| t.task_id != task_id);
                    core_load.task_count -= 1;
                }
                scheduler.total_tasks.fetch_sub(1, Ordering::SeqCst);
            }

            Ok(())
        }
    }

    /// Get current core loads
    ///
    /// This function returns the current load of each core.
    ///
    /// # Returns
    ///
    /// * `Result<Vec<(u32, f32)>, HalError>` - A result containing the core loads or an error.
    pub fn get_core_loads() -> Result<Vec<(u32, f32)>, HalError> {
        unsafe {
            let scheduler = HYBRID_SCHEDULER.as_ref().ok_or(HalError::NotInitialized)?;
            if !scheduler.initialized.load(Ordering::SeqCst) {
                return Err(HalError::NotInitialized);
            }

            Ok(scheduler.core_loads.iter()
                .map(|c| (c.core_id, c.utilization))
                .collect())
        }
    }
}

/// Select target core for new task
///
/// This function selects the target core for a new task based on various factors such as core type, load balancing, and power efficiency.
///
/// # Arguments
///
/// * `scheduler` - A reference to the hybrid scheduler.
/// * `profile` - A reference to the task profile.
///
/// # Returns
///
/// * `Result<u32, HalError>` - A result containing the target core ID or an error.
fn select_target_core(
    scheduler: &HybridScheduler,
    profile: &TaskProfile,
) -> Result<u32, HalError> {
    // Get current power policy
    let policy_mode = PolicyManager::get_system_state()?;

    // Adjust P-core preference based on power policy
    let p_core_bias = match policy_mode {
        PolicyMode::Performance => scheduler.config.p_core_preference,
        PolicyMode::Balanced => scheduler.config.p_core_preference * 0.7,
        PolicyMode::PowerSaver => scheduler.config.p_core_preference * 0.3,
        PolicyMode::Custom => scheduler.config.p_core_preference,
    };

    // Score each core
    let mut best_score = f32::MIN;
    let mut best_core = 0;

    for core in &scheduler.core_loads {
        let mut score = 0.0;

        // Core type suitability
        match core.core_type {
            CoreType::Performance => {
                score += p_core_bias * (1.0 - core.utilization);
                if profile.cpu_intensity > 0.7 {
                    score += 0.3;
                }
            }
            CoreType::Efficiency => {
                score += (1.0 - p_core_bias) * (1.0 - core.utilization);
                if profile.io_intensity > 0.7 {
                    score += 0.3;
                }
            }
        }

        // Load balancing
        score -= (core.task_count as f32 * 0.1);

        // Power efficiency
        if policy_mode == PolicyMode::PowerSaver && core.core_type == CoreType::Efficiency {
            score += scheduler.config.power_efficiency;
        }

        // Update best core
        if score > best_score {
            best_score = score;
            best_core = core.core_id;
        }
    }

    Ok(best_core)
}

/// Evaluate if task should migrate between cores
///
/// This function evaluates if a task should migrate between cores based on its characteristics and the current core load.
///
/// # Arguments
///
/// * `scheduler` - A mutable reference to the hybrid scheduler.
/// * `profile` - A reference to the task profile.
///
/// # Returns
///
/// * `Result<(), HalError>` - A result indicating success or an error.
fn evaluate_task_migration(
    scheduler: &mut HybridScheduler,
    profile: &TaskProfile,
) -> Result<(), HalError> {
    let current_core = scheduler.core_loads.iter()
        .find(|c| c.core_id == profile.last_core)
        .ok_or(HalError::DeviceError)?;

    // Check migration conditions
    let should_migrate = match current_core.core_type {
        CoreType::Performance => {
            // Migrate to E-core if low CPU intensity
            profile.cpu_intensity < 0.3 && profile.io_intensity > 0.5
        }
        CoreType::Efficiency => {
            // Migrate to P-core if high CPU intensity
            profile.cpu_intensity > 0.7 && profile.io_intensity < 0.3
        }
    };

    if should_migrate {
        // Select new core
        let target_core = select_target_core(scheduler, profile)?;
        if target_core != profile.last_core {
            // Perform migration
            migrate_task(scheduler, profile.task_id, target_core)?;
        }
    }

    Ok(())
}

/// Migrate task between cores
///
/// This function migrates a task between cores. It removes the task from the current core and adds it to the target core.
///
/// # Arguments
///
/// * `scheduler` - A mutable reference to the hybrid scheduler.
/// * `task_id` - The ID of the task to migrate.
/// * `target_core` - The ID of the target core.
///
/// # Returns
///
/// * `Result<(), HalError>` - A result indicating success or an error.
fn migrate_task(
    scheduler: &mut HybridScheduler,
    task_id: u32,
    target_core: u32,
) -> Result<(), HalError> {
    if let Some(profile) = scheduler.task_profiles.get_mut(&task_id) {
        // Remove from current core
        if let Some(current_core) = scheduler.core_loads.iter_mut()
            .find(|c| c.core_id == profile.last_core) {
            current_core.active_tasks.retain(|t| t.task_id != task_id);
            current_core.task_count -= 1;
        }

        // Add to target core
        if let Some(target_load) = scheduler.core_loads.iter_mut()
            .find(|c| c.core_id == target_core) {
            target_load.active_tasks.push_back(profile.clone());
            target_load.task_count += 1;
            profile.last_core = target_core;
        }
    }

    Ok(())
}
