// Import necessary modules and components from various libraries
import React, { useState } from "react"; // useState is a Hook that lets you add React state to function components
import axios from "axios"; // axios is a promise-based HTTP client for the browser and node.js
import {
		Stack,
		Text,
		InputGroup,
		InputRightElement,
		Input,
		Icon,
		useToast,
		Button,
		Image,
		Box,
		Container,
		FormControl,
		FormErrorMessage,
} from "@chakra-ui/react"; // These are components from Chakra UI, a simple, modular and accessible component library
import { FaDice } from "react-icons/fa"; // FaDice is an icon from react-icons library
import dash from "./images/Dashboard.PNG"; // dash is an image imported from local directory

// Define a functional component named Home
const Home = (props) => {
		// Declare a new state variable, which we'll call "email"
		const [email, setEmail] = useState("");
		// useToast is a hook from Chakra UI that is used to display toast notifications
		const toast = useToast();

		// Define a function to handle form submission
		const handleSubmit = async (e) => {
				// Check if the email is valid using a regular expression
				let error = !email
						.toLowerCase()
						.match(
								/^(([^<>()[\]\\.,;:\s@"]+(\.[^<>()[\]\\.,;:\s@"]+)*)|(".+"))@((\[[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\])|(([a-zA-Z\-0-9]+\.)+[a-zA-Z]{2,}))$/
						);
				// If the email is invalid, show a toast notification
				error &&
						toast({
								title: "Invalid Email",
								description: "Make sure to enter a valid email address!",
								status: "error",
								duration: 3000,
								isClosable: true,
						});
				// If the email is valid, send a POST request to a specified URL with the email as data
				if (!error) {
						axios.post(
								`https://sheet.best/api/sheets/c122b525-c0e2-4ebd-997e-614116491820`,
								{ email }
						);
						// Show a success toast notification
						toast({
								title: "Success!",
								description: "We've got your email noted and will reach out soon.",
								status: "success",
								duration: 3000,
								isClosable: true,
						});
				}
		};

		// Render the component
		return (
				<>
						{/* The component consists of a section with various nested components */}
						{/* The form control contains an input group for the email input and a submit button */}
						{/* The image is displayed in a box component */}
				</>
		);
};

// Export the Home component so it can be imported and used in other files
export default Home;
