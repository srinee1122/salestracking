// src/model/api.ts

// Import the invoke function from Tauri's core API
import { invoke } from '@tauri-apps/api/core';

// --- Interfaces ---

// Interface matching the Rust Salesperson struct
// Exported so other modules (like Vue components) can use it
export interface Salesperson {
  id: number;
  name: string;
  employee_id: string | null; // Corresponds to Option<String> in Rust
  email: string | null;       // Corresponds to Option<String> in Rust
  is_active: boolean;
}

// Interface for the data structure expected by the add_salesperson Rust command
// Exported so the Vue component can create payloads of this type
export interface AddSalespersonPayload {
    name: string;
    // Ensure this key 'employeeId' exactly matches the argument name in your Rust command
    employeeId: string | null;
    email: string | null;
}

// --- API Functions ---

/**
 * Fetches the list of all salespeople from the Rust backend.
 * Calls the 'get_salespeople' Tauri command.
 * @returns A Promise resolving to an array of Salesperson objects.
 * @throws An error if the backend invocation fails.
 */
export async function apiFetchSalespeople(): Promise<Salesperson[]> {
    console.log("API: Fetching salespeople...");
    try {
        // Call the 'get_salespeople' command in Rust, expecting an array of Salesperson objects
        const people = await invoke<Salesperson[]>('get_salespeople');
        console.log(`API: Fetched ${people.length} salespeople successfully.`);
        return people;
    } catch (error) {
        // Log the error and re-throw it as a standard Error object for the caller to handle
        console.error("API Error fetching salespeople:", error);
        throw new Error(String(error)); // Convert unknown error type to string message
    }
}

/**
 * Sends data to the Rust backend to add a new salesperson.
 * Calls the 'add_salesperson' Tauri command.
 * @param payload - An object containing the new salesperson's details (name, employeeId, email).
 * @returns A Promise that resolves when the operation is complete.
 * @throws An error if the backend invocation fails.
 */
export async function apiAddSalesperson(payload: AddSalespersonPayload): Promise<void> {
    console.log("API: Adding salesperson...", payload);
    try {
        await invoke('add_salesperson', {
            name: payload.name,
            employeeId: payload.employeeId,
            email: payload.email
        });
        console.log("API: Added salesperson successfully.");
    } catch (error) {
        console.error("API Error adding salesperson:", error);
        throw new Error(String(error));
    }

}
