import * as Path from "@std/path";

const install = (
  url: URL,
  downloadDirectory: string,
  installDirectory: string,
) => {
  console.log("Starting Oracle JDK 25 installation...");

  const debianFileName = "jdk-25_linux-x64_bin.deb";
  const debianDownloadFilePath = Path.join(downloadDirectory, debianFileName);
  const debianInstallFilePath = Path.join(downloadDirectory, debianFileName);

  
  mkdir -p "$INSTALL_DIR"
};

install(
  new URL("https://download.oracle.com/java/25/latest/jdk-25_linux-x64_bin.deb"),
  "/tmp",
  "/shared"
);