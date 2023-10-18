terraform {
  required_version = "<=0.15"
}

module "gcp" {
  source = "./gcp"
}

module "azure" {
  source = "./azure"
}

module "aws" {
  source = "./aws"
}
