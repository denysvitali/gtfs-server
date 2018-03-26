node {
    checkout scm
    def customImage = docker.build("dvitali/gtfs-server:${env.BUILD_ID}")
    customImage.push()

    customImage.push('latest')
}