terraform {
  required_version = "~> 1.3"
}

module "conditionally_used_dev_module" {
  source = "./dev"
  count  = var.environment == "dev" ? 1 : 0
}

module "conditionally_used_staging_module" {
  source = "./staging"
  count  = var.environment == "staging" ? 1 : 0
}

module "conditionally_used_production_module" {
  source = "./production"
  count  = var.environment == "production" ? 1 : 0
}
