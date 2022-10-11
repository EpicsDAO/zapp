![ZAPP](https://storage.googleapis.com/zapp-bucket/img/ZAPP-logo.jpeg)
<p align="center">
  <a href="https://twitter.com/intent/follow?screen_name=EpicsDAO2">
    <img src="https://img.shields.io/twitter/follow/EpicsDAO2.svg?label=Follow%20@EpicsDAO2" alt="Follow @EpicsDAO2" />
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

<p>
  <a aria-label="Rust Serverless Application Framework">
    <img src="https://storage.googleapis.com/zapp-bucket/img/zapp-new2.gif">
  </a>
</p>

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

## Document 

- [Document](https://zapp.epics.dev/)


## Installation

```bash
$ cargo install zapp
```

```bash
$ zapp --help
zapp 0.5.6
EpicsDAO
Rust Serverless Framework

USAGE:
    zapp <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    compute    
    db         
    docker     
    gen       
    gcloud     
    gh         
    help       Print this message or the help of the given subcommand(s)
    iam        
    init       
    new        
    run        
    sql        
```

## QuickStart

Create your application with zapp CLI.
```bash
$ zapp new YOURAPP

  ███████╗ █████╗ ██████╗ ██████╗ 
  ╚══███╔╝██╔══██╗██╔══██╗██╔══██╗
    ███╔╝ ███████║██████╔╝██████╔╝
   ███╔╝  ██╔══██║██╔═══╝ ██╔═══╝ 
  ███████╗██║  ██║██║     ██║     
  ╚══════╝╚═╝  ╚═╝╚═╝     ╚═╝     
💃 💃 💃 💃 💃 💃 💃 💃 💃 💃 💃 💃 
Rust Serverless Framework
$ cd YOURAPP
$ zapp docker psql
$ cargo run

Go to : http://localhost:3000/api/graphql
```

Create Docker PostgreSQL
```bash
$ zapp docker psql
✅ PostgreSQL Container Created: 4619cfc047f3cad6c9db8d255aff841fbfe34bbef0e2661fa3a02db5d5ec5d91
```

Run GraphQL Local Server
```bash
$ cargo run
```

Now go to

[http://localhost:3000/api/graphql](http://localhost:3000/api/graphql)



## GraphQL Mutation/Query Scaffold 

Create Model
```bash
$ zapp gen model user
✅ Successfully created migration file: migration/src/m20220613_102512_create_user_table.rs
✅ Successfully added route to `migration/src/lib.rs`
✅ Successfully created entity file: entity/src/user.rs
✅ Successfully created mutation file: src/graphql/mutation/user.rs
✅ Successfully created query file: src/graphql/query/user.rs
✅ Successfully added route to `entity/src/lib.rs`
✅ Successfully added mutation route: src/graphql/mutation/mod.rs
✅ Successfully added mutation route: src/graphql/query/mod.rs
```

<p>
  <a aria-label="Async GraphQL">
    <img src="https://storage.googleapis.com/zapp-bucket/img/graphql.gif">
  </a>
</p>


- [Document](https://zapp.epics.dev/)



## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/EpicsDao/zapp. This project is intended to be a safe, welcoming space for collaboration, and contributors are expected to adhere to the [Contributor Covenant](http://contributor-covenant.org) code of conduct.

## License

The gem is available as open source under the terms of the [Apache-2.0 License](https://www.apache.org/licenses/LICENSE-2.0).

## Code of Conduct

Everyone interacting in the EpicsDAO project’s codebases, issue trackers, chat rooms and mailing lists is expected to follow the [code of conduct](https://github.com/EpicsDao/zapp/blob/master/CODE_OF_CONDUCT.md).
