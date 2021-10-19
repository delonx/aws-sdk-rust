# aws-sdk-elasticbeanstalk

**Please Note: The SDK is currently released as an alpha and is intended strictly for
feedback purposes only. Do not use this SDK for production workloads.**

AWS Elastic Beanstalk makes it easy for you to create, deploy, and manage scalable, fault-tolerant applications running on the Amazon Web Services cloud.

For more information about this product, go to the [AWS Elastic Beanstalk](http://aws.amazon.com/elasticbeanstalk/) details page. The location of the latest AWS Elastic Beanstalk WSDL is [https://elasticbeanstalk.s3.amazonaws.com/doc/2010-12-01/AWSElasticBeanstalk.wsdl](https://elasticbeanstalk.s3.amazonaws.com/doc/2010-12-01/AWSElasticBeanstalk.wsdl). To install the Software Development Kits (SDKs), Integrated Development Environment (IDE) Toolkits, and command line tools that enable you to access the API, go to [Tools for Amazon Web Services](http://aws.amazon.com/tools/).

__Endpoints__

For a list of region-specific endpoints that AWS Elastic Beanstalk supports, go to [Regions and Endpoints](https://docs.aws.amazon.com/general/latest/gr/rande.html#elasticbeanstalk_region) in the _Amazon Web Services Glossary_.

## Getting Started

> Examples are availble for many services and operations, check out the
> [examples folder in GitHub](https://github.com/awslabs/aws-sdk-rust/tree/main/sdk/examples).

The SDK provides one crate per AWS service. You must add [Tokio](https://crates.io/crates/tokio)
as a dependency within your Rust project to execute asynchronous code. To add `aws-sdk-elasticbeanstalk` to
your project, add the following to your **Cargo.toml** file:

```toml
[dependencies]
aws-config = "0.0.21-alpha"
aws-sdk-elasticbeanstalk = "0.0.21-alpha"
tokio = { version = "1", features = ["full"] }
```

## Using the SDK

Until the SDK is released, we will be adding information about using the SDK to the
[Guide](https://github.com/awslabs/aws-sdk-rust/blob/main/Guide.md). Feel free to suggest
additional sections for the guide by opening an issue and describing what you are trying to do.

## Getting Help

* [GitHub discussions](https://github.com/awslabs/aws-sdk-rust/discussions) - For ideas, RFCs & general questions
* [GitHub issues](https://github.com/awslabs/aws-sdk-rust/issues/new/choose) – For bug reports & feature requests
* [Generated Docs (latest version)](https://awslabs.github.io/aws-sdk-rust/)
* [Usage examples](https://github.com/awslabs/aws-sdk-rust/tree/main/sdk/examples)

## License

This project is licensed under the Apache-2.0 License.
