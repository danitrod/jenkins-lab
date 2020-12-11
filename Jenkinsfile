pipeline {
	agent {
		docker {
			image 'rust:alpine'
		}
	}
	options {
		skipStagesAfterUnstable()
	}
	stages {
		stage('Test') { 
			steps {
				sh 'cargo test' 
			}
		}
		stage('Build') {
			steps {
				sh 'cargo build --release'
			}
		}
		stage('Deliver') {
			steps {
				sh './scripts/deliver.sh'
			}
		}
	}
}
