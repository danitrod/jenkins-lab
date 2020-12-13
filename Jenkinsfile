pipeline {
	environment {
		imagename = "danitrod/rust-calculator"
		registryCredential = 'Dockerhub-creds' // ID of Docker credential created in Jenkins
		dockerImage = ''
	}
	agent {
		docker {
			// My image with Rust + Docker (./docker/agent-image/Dockerfile)
			image 'danitrod/rust-docker:latest'
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
			steps {
				script {
					dockerImage = docker.build imagename
				}
			}
		}
		stage('Deploy Docker image') {
			steps {
				script {
					docker.withRegistry( '', registryCredential ) {
						// Push with a number and latest
						dockerImage.push("1.$BUILD_NUMBER")
						dockerImage.push('latest')
					}
				}
			}
		}
		stage('Cleanup Docker image') {
			steps {
				sh "docker rmi $imagename:1.$BUILD_NUMBER"
				sh "docker rmi $imagename:latest"
			}
		}
	}
}
