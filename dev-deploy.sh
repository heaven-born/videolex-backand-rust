# Exit immediately if any command fails
set -e

cd main
rm -f ./target/lambda/lambda_function.zip
cargo lambda build --release --arm64
cd ./target/lambda/lambda
zip -r ../lambda_function.zip .
cd ../../../..
terraform apply -auto-approve
