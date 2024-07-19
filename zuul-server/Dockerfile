# Stage 1: Build the application
FROM maven:3.8.4-openjdk-11 AS build
WORKDIR /app
# Copy the project files into the Docker image
COPY src ./src
COPY pom.xml .
# Build the application
RUN mvn clean package -DskipTests

# Stage 2: Run the application
FROM openjdk:11
WORKDIR /app
# Copy the jar file from the build stage
COPY --from=build /app/target/apigatewayzuul-0.0.1-SNAPSHOT.war /app/zuul-server.jar
EXPOSE 8080
# Run the application
CMD ["java", "-jar", "zuul-server.jar"]

