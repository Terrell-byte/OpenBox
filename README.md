# OpenBox
<p align="center">
  <img src="https://github.com/Terrell-byte/OpenBox/blob/main/OpenBox-logo.png" />
</p>
<p align="center">
    <a href="https://github.com/Terrell-byte/OpenBox/pulse">
      <img src="https://img.shields.io/github/last-commit/Terrell-byte/OpenBox?style=for-the-badge&logo=github&color=7dc4e4&logoColor=D9E0EE&labelColor=302D41"/>
    </a>
    <a href="https://github.com/Terrell-byte/OpenBox/releases/latest">
      <img src="https://img.shields.io/github/v/release/Terrell-byte/OpenBox?style=for-the-badge&logo=gitbook&color=8bd5ca&logoColor=D9E0EE&labelColor=302D41"/>
    </a>
    <a href="https://github.com/Terrell-byte/OpenBox/stargazers">
      <img src="https://img.shields.io/github/stars/Terrell-byte/OpenBox?style=for-the-badge&logo=apachespark&color=eed49f&logoColor=D9E0EE&labelColor=302D41"/>
    </a>
</p>

## Introduction
OpenBox offers an alternative to paid solutions for individuals looking to build or manage a home server. It functions similarly to Dropbox, providing an easy and efficient way to handle your server needs at home without the cost.

## Features
- **Ease of Use**: Drag and drop files directly into your server.
- **Simple Server Deployment**: Easily deploy your server side with Docker.
- **User-Friendly Interface**: Developed using WPF .NET for a smooth user experience.

## Installation
**Server Side:**
```shell
git clone https://github.com/Terrell-byte/OpenBox
cd OpenBox/
git config core.sparseCheckout true
echo "server/*" >> .git/info/sparse-checkout
git checkout
```
**Client Side:**
Download the latest release .exe file from our releases page.

## Usage
After installation, connect to your server's IP address. The system will prompt you to create a new admin profile. As an admin, you can create new user accounts for family and friends and assign permissions as needed.

## Contributing
Interested in contributing to OpenBox? Great! Please submit a pull request with details of your changes. We appreciate your efforts to improve the project.

## License
OpenBox is available under the MIT License. For more details, see the LICENSE file in the repository.

## Contact
For any queries or support requests, please open an issue on the repository.

## Acknowledgments
Special thanks to all the contributors who have made OpenBox what it is today.
- **Terrell-byte**
