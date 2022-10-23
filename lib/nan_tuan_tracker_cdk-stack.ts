import { Stack, StackProps, aws_lambda, aws_dynamodb, aws_iam, Duration, aws_events_targets, aws_events } from 'aws-cdk-lib';
import { Schedule } from 'aws-cdk-lib/aws-events';
import { ManagedPolicy } from 'aws-cdk-lib/aws-iam';
import { RetentionDays } from 'aws-cdk-lib/aws-logs';
import { Construct } from 'constructs';

export class NanTuanTrackerCdkStack extends Stack {
  constructor(scope: Construct, id: string, props?: StackProps) {
    super(scope, id, props);

    // TODO: Adding CloudWatch EventBridge to periodically trigger the lambda

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
        STRATZ_JWT: 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJuYW1laWQiOiJodHRwczovL3N0ZWFtY29tbXVuaXR5LmNvbS9vcGVuaWQvaWQvNzY1NjExOTgxNTYxMzIyMDMiLCJ1bmlxdWVfbmFtZSI6IllvdGVsbCIsIlN1YmplY3QiOiJjZDNmMTJlOS1iMTg2LTRjNjMtYTg4NC1iZmE0NjQxMjg4OWMiLCJTdGVhbUlkIjoiMTk1ODY2NDc1IiwibmJmIjoxNjM2NzAwNjE2LCJleHAiOjE2NjgyMzY2MTYsImlhdCI6MTYzNjcwMDYxNiwiaXNzIjoiaHR0cHM6Ly9hcGkuc3RyYXR6LmNvbSJ9.znoKKTrxQB1BtnxH5zsf8oitr6jj_vN8rm8Dr6NyFWQ',
        DISCORD_WEBHOOK_URL: 'https://discord.com/api/webhooks/1031066435075702824/gEnNJ2960J02TkOYQGhN1baIe5uDHBltYv5Vd5H4NUmY4B-uj6Ozr9DZTwqtb5PN3DAD',
      },
      logRetention: RetentionDays.ONE_DAY,
      role: lambdaRole,
      timeout: Duration.seconds(30)
    });

    const lambdaEventRule = new aws_events.Rule(this, 'lambdaScheduleRule', {
      schedule: Schedule.rate(Duration.minutes(5)),
      targets: [new aws_events_targets.LambdaFunction(pollerLambda)]
    });

    const guildIdTable = new aws_dynamodb.Table(this , "GuildIdTable", {
      tableName: 'Guilds',
      billingMode: aws_dynamodb.BillingMode.PROVISIONED,
      readCapacity: 5,
      writeCapacity: 10,
      partitionKey: {
        name: 'id', 
        type: aws_dynamodb.AttributeType.NUMBER
      }
    });
  }
}
