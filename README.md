# NanTuanTracker

## Description
- Periodically poll dota2 match results from NanTuan guild using Stratz GraphQL API.  [Powered by STRATZ](https://stratz.com/)
- After polling the data, we will parse them and publish to different destinations.
    - [X] Discord
    - [X] Kook

## Tech stacks
![AWS](https://img.shields.io/badge/Amazon_AWS-232F3E?style=for-the-badge&logo=amazon-aws&logoColor=white)
![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white) 
![TypeScript](https://img.shields.io/badge/typescript-%23007ACC.svg?style=for-the-badge&logo=typescript&logoColor=white)
- Using AWS CloudFormation CDK to "Infrastructure as Code"
- Using AWS DynamoDB to keep track of matches polled, so no duplicated matches will be published. 
- Using AWS Lambda to host serverless function.
- Using AWS CloudWatch EventBridges to trigger AWS Lambda periodically.
- Rust (Service) + TypeScript (Infrastructure)

## Development
Here are commands to test infrastructure codes
* `npm run build`       compile typescript to js
* `npm run watch`       compile and enter the watch mode
* `npm run test`        perform the jest unit tests

Here is the command to build Rust Lambda function to AL2 runtime
* `./build-poller.sh`   compile Rust lambda to AL2 runtime
*NOTE*: The AL2 runtime might be different from your local machine runtime, so the binary might not be executable.

Here are commands for AWS cdk
* `cdk diff`            compare deployed stack with current state
* `cdk synth`           emits the synthesized CloudFormation template

Here is the one command that will wrap up everything and deploy the function
* `npm run deploy`      deploy this stack to your default AWS account/region

More details here: docs.aws.amazon.com/cdk/v2/guide/work-with-cdk-typescript.html
