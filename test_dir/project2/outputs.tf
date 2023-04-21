output "module_used" {
  value = concat(module.conditionally_used_dev_module[*].vpc_id, module.conditionally_used_production_module[*].vpc_id, module.conditionally_used_staging_module[*].vpc_id)
}
