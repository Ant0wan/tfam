terraform {
  required_version = "1.3"
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
