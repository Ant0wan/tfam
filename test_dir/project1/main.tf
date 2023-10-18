terraform {
  required_version = "<=0.15.2"
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
