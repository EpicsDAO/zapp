pub async fn action_yml(gcr_region: &str) -> String {
  let yml = format!("name: ZappService
on:
  push:
    branches:
      - main

jobs:
  build:
    runs-on: ubuntu-20.04
    steps:
    - name: Checkout the repository
      uses: actions/checkout@v2

    - id: auth
      uses: google-github-actions/auth@v0
      with:
        credentials_json: ${{ secrets.ZAPP_GCP_SA_KEY }}

    - name: Set up Cloud SDK
      uses: google-github-actions/setup-gcloud@v0
      with:
        install_components: 'beta'
    
    - name: Configure Docker
      run: gcloud auth configure-docker --quiet
    
    - name: Build Docker container
      run: docker build . -t {}/${{ secrets.ZAPP_GCP_PROJECT_ID }}/${{secrets.ZAPP_SERVICE_NAME}}

    - name: Push to Container Resistory
      run: docker push {}/${{ secrets.ZAPP_GCP_PROJECT_ID }}/${{secrets.ZAPP_SERVICE_NAME}}

    - name: Deploy to Cloud Run
      run: |
          gcloud beta run deploy zapp-${{ secrets.ZAPP_SERVICE_NAME }} \
            --quiet \
            --service-account=${{ secrets.ZAPP_SERVICE_NAME }}@${{ secrets.ZAPP_GCP_PROJECT_ID }}.iam.gserviceaccount.com \
            --image={}/${{ secrets.ZAPP_GCP_PROJECT_ID }}/${{ secrets.ZAPP_SERVICE_NAME }} \
            --memory=8Gi \
            --cpu=2 \
            --vpc-connector='${{ secrets.ZAPP_SERVICE_NAME }}' \
            --vpc-egress=all \
            --region=${{ secrets.ZAPP_GCP_REGION }} \
            --allow-unauthenticated \
            --platform=managed \
            --no-cpu-throttling \
            --execution-environment=gen2 \
            --set-cloudsql-instances=${{ secrets.ZAPP_GCLOUDSQL_INSTANCE }} \
            --concurrency=80 \
            --port=8080 \
            --set-env-vars='ZAPP_GCP_PROJECT_ID=${{ secrets.ZAPP_GCP_PROJECT_ID }}' \
            --set-env-vars='DATABASE_URL=${{ secrets.DATABASE_URL }}'
", gcr_region, gcr_region, gcr_region);
yml
}
