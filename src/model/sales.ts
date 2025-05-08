import { invoke } from "@tauri-apps/api/core";


// Type matching the SaleEntry struct in Rust
export interface SaleEntry {
  id: number;
  salesperson_id: number;
  date: string;
  product_id: number;
  quantity: number;
  unit_type: "pieces" | "cartons";
}

// Payload to send when creating a new sale
export interface NewSaleEntry {
  salesperson_id: number;
  date: string;
  product_id: number;
  quantity: number;
  unit_type: "pieces" | "cartons";
}

// Call Rust to add a new sale entry
export async function apiAddSaleEntry(payload: NewSaleEntry): Promise<void> {
  console.log("API: Adding sale entry...", payload);
  try {
    await invoke("add_sale_entry", { entry: payload });
    console.log("✅ Sale entry added successfully.");
  } catch (error) {
    console.error("❌ API Error adding sale entry:", error);
    throw new Error(String(error));
  }
}



// Call Rust to fetch all sale entries
export async function apiFetchSaleEntries(): Promise<SaleEntry[]> {
  console.log("API: Fetching sale entries...");
  try {
    const sales = await invoke<SaleEntry[]>("get_sales_entries");
    console.log("✅ Sale entries fetched:", sales);
    return sales;
  } catch (error) {
    console.error("❌ API Error fetching sale entries:", error);
    throw new Error(String(error));
  }
}
