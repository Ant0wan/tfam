<br />
<p align="center">
  <a href="">
    <img src=".tflogo.png" alt="Logo" height="80">
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

## License

This repository is protected by the GPL3 (GNU General Public License v3.0). You can find the full text of the license in the LICENSE file. Please review and comply with the terms and conditions of the GPL3 license before using or contributing to this project.

For any questions, bug reports, or contributions, please feel free to open an issue or submit a pull request. Thank you for using tfam!
