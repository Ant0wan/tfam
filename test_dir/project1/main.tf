terraform {
  required_version = "1.4"
}

module "gcp" {
  source "./gcp"
}
  
module "azure" {
  source "./azure"
}
  
module "aws" {
  source "./aws"
}
