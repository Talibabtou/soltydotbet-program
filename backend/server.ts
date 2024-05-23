// Importing necessary modules
import dotenv from 'dotenv';
import express, { Request, Response, NextFunction } from 'express';
import cors from 'cors';
import mongoose from 'mongoose';
import userRoutes from './routes/user';
import path from 'path';

// Load environment variables from .env file
dotenv.config();

// Create an Express application
const app = express();

// Use JSON middleware to automatically parse JSON
app.use(express.json());

// Use CORS middleware to handle Cross Origin Resource Sharing
app.use(cors());

// Custom middleware to log request path and method
app.use((req: Request, res: Response, next: NextFunction) => {
	console.log(req.path, req.method);
	next();
});

// Use userRoutes for handling routes starting with "/api/users"
app.use("/api/users", userRoutes);

// Define a GET route at the root ("/") of userRoutes
// This route fetches all users from the database and sends them back as a JSON response
userRoutes.route("/").get(function (req: Request, res: Response) {
	User.find(function (err: any, user: any) {
		if (err) {
			console.log(err);
		} else {
			res.json(user);
		}
	});
});

// Serve frontend
const __dirname = path.resolve();
if (process.env.NODE_ENV === "production") {
	app.use(express.static(path.join(__dirname, "../frontend/build")));

	app.get("*", (req: Request, res: Response) =>
		res.sendFile(
			path.resolve(__dirname, "../", "frontend", "build", "index.html")
		)
	);
} else {
	app.get("/", (req: Request, res: Response) => res.send("Please set to production"));
}

// Connect to MongoDB database
mongoose
	.connect(process.env.MONGO_URI as string)
	.then(() => {
		// Start the server and listen for requests
		app.listen(process.env.PORT, () => {
			console.log(`listening on port ${process.env.PORT}!`);
		});
	})
	.catch((error: any) => {
		console.log(error);
	});
