<br />
<p align="center">
  <a href="">
    <img src=".logo.png" alt="Logo" height="80">
  </a>
      <h1 align="center">Terraform Apply Manager</h1>
  <p align="center"><i>A Rust-based wrapper for concurrent Terraform apply, enabling multi-deployment support.</i></p>
</p>

---

*tfam* stands for *"Terraform Apply Manager"*


## How it works

```mermaid
flowchart TD
    A(tfam) --> C{dev.tfvars}
    A(tfam) --> E{staging.tfvars}
    A(tfam) --> F{production.tfvars}
    C  --> ter(terraform)
    E  --> te(terraform)
    F  --> t(terraform)
```

## Specifications

Terraform Apply Manager interacts with the Terraform binary and provides the following features:

- **Non-invasive**: It reads and uses `.tf` and `.tfvars` files as they are, even if they have been modified by the user. This ensures that the user maintains full control over their configuration files and prevents interference with other processes that may also be using those files.
- **Non-destructive**: It preserves any Terraform options and commands that are passed to it, and passes them on to the Terraform binary unchanged. This ensures that the behavior of the Terraform binary remains the same, and that any options or commands provided by the user are respected. The tool/script may add additional functionality, but it should not modify or remove any existing functionality.

By providing these features, Terraform Apply Manager offers a seamless and flexible way to interact with the Terraform binary, without interfering with the user's workflow or modifying the behavior of the binary.

## Usage

`TF_WORKSPACE_FORMAT`: awk-like format for workspace from file path $1 = $n-1, $2 = $n-2 `-workspace-format='hello$2_$1_$3'
`export TF_WORKSPACE_FORMAT='$-3_$-1_$-2'`
or
`tfam -interactive -workspace-format='$-3_$-1_$-2`

`-interactive`: selection menu

`-concurrent`:

`-var-file`: multiple


## License

This repository is protected by the GPL3 (GNU General Public License v3.0). You can find the full text of the license in the LICENSE file. Please review and comply with the terms and conditions of the GPL3 license before using or contributing to this project.

For any questions, bug reports, or contributions, please feel free to open an issue or submit a pull request. Thank you for using tfam!
