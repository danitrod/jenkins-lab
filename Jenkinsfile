pipeline {
	environment {
    imagename = "danitrod/rust-calculator"
    registryCredential = 'danitrod'
    dockerImage = ''
  }
	agent any
	options {
		skipStagesAfterUnstable()
	}
	stages {
		stage('Test app') { 
			steps {
				sh 'cargo test' 
			}
		}
		stage('Build app') {
			steps {
				sh 'cargo build --release'
			}
		}
		stage('Build Docker image') {
			steps{
        script {
          dockerImage = docker.build imagename
        }
      }
		}
		stage('Deploy Docker image') {
      steps{
        script {
          docker.withRegistry( '', registryCredential ) {
            dockerImage.push("$BUILD_NUMBER")
             dockerImage.push('latest')
          }
        }
      }
    }
		stage('Cleanup Docker image') {
      steps{
        sh "docker rmi $imagename:$BUILD_NUMBER"
        sh "docker rmi $imagename:latest"
      }
    }
	}
}
