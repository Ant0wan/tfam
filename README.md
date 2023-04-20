<br />
<p align="center">
  <a href="">
    <img src=".logo.png" alt="Logo" height="80">
  </a>
      <h1 align="center">Terraform Apply Manager</h1>
  <p align="center"><i>A Rust-based wrapper for concurrent Terraform apply, enabling multi-deployment support.</i></p>
</p>

---

*tfam* for *"Terraform Apply Manager"*


### How it works

```mermaid
flowchart TD
    A(tfam) --> C{dev.tfvars}
    A(tfam) --> E{staging.tfvars}
    A(tfam) --> F{production.tfvars}
    C  --> ter(terraform)
    E  --> te(terraform)
    F  --> t(terraform)
```
