terraform {
  required_version = "~> 1.24"
}

module "conditionally_used_env_module" {
  source = "./${var.environment}"
}

