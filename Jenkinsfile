pipeline {
	environment {
    imagename = "danitrod/rust-calculator"
    registryCredential = 'danitrod-dockerhub'
    dockerImage = ''
  }
	agent {
		docker {
			image 'rust:alpine'
			args '--user root -v /var/run/docker.sock:/var/run/docker.sock -v /usr/bin/docker:/usr/bin/docker'
		}
	}
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
