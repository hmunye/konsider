# Konsider Client (Standalone)

## Usage

### 1. **Prerequisites**

Before starting, ensure the following tools are installed:

- **Node.js**: Install Node.js by following the instructions on [Download Node.js](https://nodejs.org/en/download/package-manager).

### 2. **Clone the Repository**

```bash
git clone https://github.com/hmunye/konsider.git
```
```bash
cd konsider/client
```

### 3. Install Dependencies

```bash
npm install
```

### 4. Set Up Environment Variables

Make sure you have the `.env.production` file in the `client` directory. 
If it does not exist, create it and add the following configuration:

```bash
touch ./.env.production
```

```bash
PUBLIC_BASE_API_URL=https://<server-ip>
```

Replace `<server-ip>` with the actual IP address or domain of the backend server

### 5. Start the Application

Once dependencies are installed and the environment variables are set up, 
start the SvelteKit application in development mode by running:

```bash
npm run dev
```

This will start the application, and you can access it at `http://localhost:3080`.

### 5. Build for Production

To create a production build of the application, run:

```bash
npm run build
```

This will generate a production-ready version of the application.

To preview the production build locally, run:

```bash
npm run preview
```
