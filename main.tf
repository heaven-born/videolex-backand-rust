
locals {
  videoloex_backand = "videolex-backend"
}

variable "OPENAI_API_KEY" {}

provider "aws" {
  region = "us-east-1" # Change to your desired region
}

resource "aws_iam_role" "lambda_exec_role" {
  name = "lambda_exec_role_${local.videoloex_backand}"

  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Action = "sts:AssumeRole"
        Effect = "Allow"
        Principal = {
          Service = "lambda.amazonaws.com"
        }
      }
    ]
  })
}

# IAM Policy for ECR Access and CloudWatch Logs
resource "aws_iam_role_policy" "lambda_ecr_access" {
  name = "lambda_${local.videoloex_backand}_ecr_access_policy"
  role = aws_iam_role.lambda_exec_role.id

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Effect = "Allow"
        Action = [
          "ecr:GetDownloadUrlForLayer",
          "ecr:BatchGetImage",
          "ecr:GetAuthorizationToken"
        ]
        Resource = "*"
      },
      {
        Effect = "Allow"
        Action = [
          "logs:CreateLogGroup",
          "logs:CreateLogStream",
          "logs:PutLogEvents"
        ]
        Resource = "arn:aws:logs:*:*:*"
      }
    ]
  })
}

resource "aws_lambda_function" "videolex_backend" {
  function_name = "${local.videoloex_backand}-lambda"
  role          = aws_iam_role.lambda_exec_role.arn
  filename      = "./main/target/lambda/lambda_function.zip"
  source_code_hash = filebase64sha256("./main/target/lambda/lambda_function.zip")
  handler       = "bootstrap"
  runtime       = "provided.al2023"
  timeout       = 60
  architectures = ["arm64"]
  environment {
    variables = {
      "OPENAI_API_KEY" = var.OPENAI_API_KEY
    }
  }
}

resource "aws_lambda_permission" "allow_invoke" {
  statement_id  = "AllowExecutionFromAPI"
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.videolex_backend.function_name
  principal     = "apigateway.amazonaws.com"
}

# API Gateway REST API
resource "aws_api_gateway_rest_api" "videolex_backend_api" {
  name        = "${local.videoloex_backand}-api"
  description = "API Gateway for ${local.videoloex_backand} Lambda function"
  endpoint_configuration {
    types = ["REGIONAL"]
  }
}

# API Gateway Resource: guess-words
resource "aws_api_gateway_resource" "guess_words_resource" {
  rest_api_id = aws_api_gateway_rest_api.videolex_backend_api.id
  parent_id   = aws_api_gateway_rest_api.videolex_backend_api.root_resource_id
  path_part   = "explain-word"
}

# API Gateway Method: POST /guess-words
resource "aws_api_gateway_method" "guess_words_method" {
  rest_api_id   = aws_api_gateway_rest_api.videolex_backend_api.id
  resource_id   = aws_api_gateway_resource.guess_words_resource.id
  http_method   = "POST"
  authorization = "NONE"
}

resource "aws_api_gateway_integration" "guess_words_integration" {
  rest_api_id             = aws_api_gateway_rest_api.videolex_backend_api.id
  resource_id             = aws_api_gateway_resource.guess_words_resource.id
  http_method             = aws_api_gateway_method.guess_words_method.http_method
  integration_http_method = "POST"
  type                    = "AWS_PROXY"
  uri                     = aws_lambda_function.videolex_backend.invoke_arn
}


resource "aws_api_gateway_resource" "tts_resource" {
  rest_api_id = aws_api_gateway_rest_api.videolex_backend_api.id
  parent_id   = aws_api_gateway_rest_api.videolex_backend_api.root_resource_id
  path_part   = "text-to-speech"
}

resource "aws_api_gateway_method" "tts_method" {
  rest_api_id   = aws_api_gateway_rest_api.videolex_backend_api.id
  resource_id   = aws_api_gateway_resource.tts_resource.id
  http_method   = "POST"
  authorization = "NONE"
}

resource "aws_api_gateway_integration" "tts_integration" {
  rest_api_id             = aws_api_gateway_rest_api.videolex_backend_api.id
  resource_id             = aws_api_gateway_resource.tts_resource.id
  http_method             = aws_api_gateway_method.tts_method.http_method
  integration_http_method = "POST"
  type                    = "AWS_PROXY"
  uri                     = aws_lambda_function.videolex_backend.invoke_arn
}



resource "aws_api_gateway_resource" "swagger_resource" {
  rest_api_id = aws_api_gateway_rest_api.videolex_backend_api.id
  parent_id   = aws_api_gateway_rest_api.videolex_backend_api.root_resource_id
  path_part   = "openapi.json"
}

resource "aws_api_gateway_method" "swagger_method" {
  rest_api_id   = aws_api_gateway_rest_api.videolex_backend_api.id
  resource_id   = aws_api_gateway_resource.swagger_resource.id
  http_method   = "GET"
  authorization = "NONE"
}

resource "aws_api_gateway_integration" "swagger_integration" {
  rest_api_id             = aws_api_gateway_rest_api.videolex_backend_api.id
  resource_id             = aws_api_gateway_resource.swagger_resource.id
  http_method             = aws_api_gateway_method.swagger_method.http_method
  integration_http_method = "POST"
  type                    = "AWS_PROXY"
  uri                     = aws_lambda_function.videolex_backend.invoke_arn
}


resource "aws_api_gateway_resource" "word_card_resource" {
  rest_api_id = aws_api_gateway_rest_api.videolex_backend_api.id
  parent_id   = aws_api_gateway_rest_api.videolex_backend_api.root_resource_id
  path_part   = "word-card"
}

resource "aws_api_gateway_method" "word_card_method" {
  rest_api_id   = aws_api_gateway_rest_api.videolex_backend_api.id
  resource_id   = aws_api_gateway_resource.word_card_resource.id
  http_method   = "POST"
  authorization = "NONE"
}

resource "aws_api_gateway_integration" "word_card_integration" {
  rest_api_id             = aws_api_gateway_rest_api.videolex_backend_api.id
  resource_id             = aws_api_gateway_resource.word_card_resource.id
  http_method             = aws_api_gateway_method.word_card_method.http_method
  integration_http_method = "POST"
  type                    = "AWS_PROXY"
  uri                     = aws_lambda_function.videolex_backend.invoke_arn
}




# API Gateway Deployment
resource "aws_api_gateway_deployment" "videolex_backend_deployment" {
  rest_api_id = aws_api_gateway_rest_api.videolex_backend_api.id
  stage_name  = "prod"
  depends_on = [
    aws_api_gateway_integration.guess_words_integration,
    aws_api_gateway_integration.tts_integration,
    aws_api_gateway_integration.swagger_integration,
    aws_api_gateway_integration.word_card_integration,
  ]
}



output "lambda_arn" {
  value = aws_lambda_function.videolex_backend.arn
}

output "api_gateway_invoke_url" {
  value = aws_api_gateway_deployment.videolex_backend_deployment.invoke_url
}

