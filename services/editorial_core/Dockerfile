FROM maven:3.9-eclipse-temurin-21 AS build

WORKDIR /usr/src/app

COPY pom.xml ./
RUN mvn dependency:go-offline -B -q

COPY src ./src
RUN mvn clean package -DskipTests -B -q

RUN java -Djarmode=tools -jar target/editorial-core-*.jar extract --layers --launcher --destination extracted

FROM eclipse-temurin:21-jre-alpine

RUN apk upgrade --no-cache && apk add --no-cache curl

RUN addgroup -S appuser \
    && adduser -S -G appuser -h /app -s /sbin/nologin appuser

WORKDIR /app

COPY --from=build --chown=appuser:appuser /usr/src/app/extracted/dependencies/ ./
COPY --from=build --chown=appuser:appuser /usr/src/app/extracted/spring-boot-loader/ ./
COPY --from=build --chown=appuser:appuser /usr/src/app/extracted/snapshot-dependencies/ ./
COPY --from=build --chown=appuser:appuser /usr/src/app/extracted/application/ ./

EXPOSE 8080

USER appuser

HEALTHCHECK --interval=30s --timeout=3s --start-period=45s --retries=3 \
    CMD curl -f http://localhost:8080/actuator/health || exit 1

ENV SPRING_PROFILES_ACTIVE=prod \
    JAVA_OPTS="-XX:+UseContainerSupport -XX:MaxRAMPercentage=75.0 -XX:+UseG1GC -XX:+ExitOnOutOfMemoryError"

ENTRYPOINT ["sh", "-c", "exec java $JAVA_OPTS org.springframework.boot.loader.launch.JarLauncher"]
