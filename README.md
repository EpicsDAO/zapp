<p align="center">
  <a href="https://twitter.com/intent/follow?screen_name=EpicsDAO">
    <img src="https://img.shields.io/twitter/follow/EpicsDAO.svg?label=Follow%20@EpicsDAO" alt="Follow @EpicsDAO" />
  </a>
  <br/>
  <a aria-label="Crate Version" href="https://crates.io/crates/zapp">
    <img alt="" src="https://badgen.net/crates/v/zapp">
  </a>
  <a aria-label="Crate Download" href="https://crates.io/crates/zapp">
    <img alt="" src="https://badgen.net/crates/d/zapp">
  </a>
  <a aria-label="License" href="https://github.com/EpicsDao/epics/blob/master/LICENSE.txt">
    <img alt="" src="https://badgen.net/badge/license/Apache/blue">
  </a>
    <a aria-label="Code of Conduct" href="https://github.com/EpicsDao/epics/blob/master/CODE_OF_CONDUCT.md">
    <img alt="" src="https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg">
  </a>
</p>

# Zapp

Rust Serverless Framework

The Zapp project was launched with the goal of reducing software development, operation and maintenance costs. 

Build Serverless Apps faster.
Powered by Rust, Aysnc-GraphQL, SeaORM, Axum, and Google Cloud. 

- Focus on business logic in serverless environment
- Maximize development efficiency with CI / CD standard schema-driven Scaffold
- Achieve global scale with lower management costs


## Dependency

- [Google SDK](https://cloud.google.com/sdk/docs)
- [Docker](https://www.docker.com/)
- [Github CLI](https://cli.github.com/)
- [SeaORM](https://www.sea-ql.org/SeaORM/)
- [Axum](https://github.com/tokio-rs/axum)
- [Async GraphQL](https://github.com/async-graphql/async-graphql)

## Cloud Infrastructure

- [Google Cloud Run](https://cloud.google.com/run)
- [Google Cloud SQL](https://cloud.google.com/sql)
- [Google Cloud IAM](https://cloud.google.com/iam)
- [Google Cloud Container Registry](https://cloud.google.com/container-registry)
- [Google Cloud VPC](https://cloud.google.com/vpc)
- [Google Cloud Nat](https://cloud.google.com/nat)
- [Github Actions](https://github.com/features/actions)


## Installation

```bash
cargo install zapp
```

### Github CLI Auth Login

```bash
gh auth login
```

### Gcloud Auth Login

```bash
gcloud auth login
```


## Deploy to Google Cloud Run

### 1. Generate Your Application
```bash
zapp new YOURAPP
cd YOURAPP
```

## 2. Create GitHub

Let's create a new repository on GitHub.

[GitHub link](https://github.com)

After creating a new repository on the above site, upload the source code with the following command.

Run from the mother directory.

```bash
$ git add .
$ git commit -m 'first commit'
$ git remote add origin git@github.com:YOURREPO/YOURAPP.git
$ git push origin main
```

### 3. Create A Google Cloud Project

If you have never used Google Cloud before, use this link to create a project.

[How to create a project](https://cloud.google.com/resource-manager/docs/creating-managing-projects)

:::div{.success}
Whether you've used Google Cloud before or it's your first time, you can get $200 free credit by using the link below:

[Google Cloud Credit](https://cloud.google.com/partners/partnercredit?pcn_code=0014M00001h3BjPQAU)
:::


### 4. Setup Cloud Compute Network
```bash
zapp compute setup
```

### 5. Push it to Github
```bash
git add .
git commit -m 'first deploy'
git push origin main
```


## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/EpicsDao/zapp. This project is intended to be a safe, welcoming space for collaboration, and contributors are expected to adhere to the [Contributor Covenant](http://contributor-covenant.org) code of conduct.

## License

The gem is available as open source under the terms of the [Apache-2.0 License](https://www.apache.org/licenses/LICENSE-2.0).

## Code of Conduct

Everyone interacting in the EpicsDAO project’s codebases, issue trackers, chat rooms and mailing lists is expected to follow the [code of conduct](https://github.com/EpicsDao/zapp/blob/master/CODE_OF_CONDUCT.md).
