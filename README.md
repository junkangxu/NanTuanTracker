# NanTuanTracker

## Description
- Periodically poll dota2 match results from NanTuan guild using Stratz GraphQL API.  [Powered by STRATZ](https://stratz.com/)
- After polling the data, we will parse them and publish to different destinations.
    - [X] Discord
    - [ ] Kook

## Tech stacks
- Using AWS DynamoDB to keep track of matches polled
- Using AWS Lambda to host serverless function
- Using AWS CloudWatch EventBridges to trigger AWS Lambda periodically
- Rust + TypeScript

## Useful commands
* `npm run build`       compile typescript to js
* `npm run watch`       watch for changes and compile
* `npm run test`        perform the jest unit tests
* `npm run deploy`      deploy this stack to your default AWS account/region
* `cdk diff`            compare deployed stack with current state
* `cdk synth`           emits the synthesized CloudFormation template
* `./build-poller.sh`   compile Rust lambda to AL2 runtime
