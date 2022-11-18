import { Stack, StackProps, aws_lambda, aws_dynamodb, aws_iam, Duration, aws_events_targets, aws_events } from 'aws-cdk-lib';
import { Schedule } from 'aws-cdk-lib/aws-events';
import { ManagedPolicy } from 'aws-cdk-lib/aws-iam';
import { RetentionDays } from 'aws-cdk-lib/aws-logs';
import { Construct } from 'constructs';

export class NanTuanTrackerCdkStack extends Stack {
  constructor(scope: Construct, id: string, props?: StackProps) {
    super(scope, id, props);

    const lambdaRole = new aws_iam.Role(this, 'scannerLambdaRole', {
      assumedBy: new aws_iam.ServicePrincipal('lambda.amazonaws.com')
    });

    lambdaRole.addManagedPolicy(
      ManagedPolicy.fromAwsManagedPolicyName('AmazonDynamoDBFullAccess')
    );

    const pollerLambda = new aws_lambda.Function(this, "PollerLambda", {
      description: 'Poller Lambda',
      code: aws_lambda.Code.fromAsset(
        'poller/target/x86_64-unknown-linux-musl/release/lambda'
      ),
      runtime: aws_lambda.Runtime.PROVIDED_AL2,
      architecture: aws_lambda.Architecture.X86_64,
      handler: 'not.required',
      environment: {
        RUST_BACKTRACE: '1',
        STRATZ_JWT: '<insert STRATZ_JWT here>',
        DISCORD_WEBHOOK_URL: '<insert DISCORD_WEBHOOK_URL here>',
        KOOK_TOKEN: '<insert KOOK_TOKEN here>',
      },
      logRetention: RetentionDays.ONE_DAY,
      role: lambdaRole,
      timeout: Duration.seconds(30)
    });

    const lambdaEventRule = new aws_events.Rule(this, 'lambdaScheduleRule', {
      schedule: Schedule.rate(Duration.minutes(2)),
      targets: [new aws_events_targets.LambdaFunction(pollerLambda)]
    });

    const guildIdTable = new aws_dynamodb.Table(this , "GuildIdTable", {
      tableName: 'Guilds',
      billingMode: aws_dynamodb.BillingMode.PROVISIONED,
      readCapacity: 2,
      writeCapacity: 2,
      partitionKey: {
        name: 'id', 
        type: aws_dynamodb.AttributeType.NUMBER
      }
    });
  }
}
