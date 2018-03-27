node {
    stage('Checkout') {
        checkout scm
    }

    stage('Build Image'){
        def customImage = docker.build("dvitali/gtfs-server:latest")
    }

    stage('Push Image') {
        def BUILD_VERSION = "sh cat Cargo.toml | grep package -A 5 | grep version | awk -F ' = ' '{print \$2}' | sed 's/\"//g'"
        withCredentials([usernamePassword(
            credentialsId: "docker-hub-dvitali",
            usernameVariable: "USER",
            passwordVariable: "PASS"
        )]) {
            sh "docker login -u $USER -p $PASS"
        }

        sh "docker tag vitali/gtfs-server:latest dvitali/gtfs-server:$BUILD_NUMBER"
        
        sh "docker push dvitali/gtfs-server:latest"
        sh "docker push dvitali/gtfs-server:$BUILD_NUMBER"
    }
}