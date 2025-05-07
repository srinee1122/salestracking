// src/model/products.ts

import { invoke } from '@tauri-apps/api/core';

// --- Interfaces ---

export interface Product {
  id: number;
  sku: string;
  name: string;
  brand: string;
  category: string;
  cost_price: number;
  unit_price: number;
  description: string | null;
}

export interface ProductPayload {
  sku: string;
  name: string;
  brand: string;
  category: string;
  cost_price: number;
  unit_price: number;
  description?: string | null;
}

// --- API Functions ---

export async function apiFetchProducts(): Promise<Product[]> {
  console.log("API: Fetching products...");
  try {
    const products = await invoke<Product[]>('get_products');
    console.log("API: Products fetched:", products);
    return products;
  } catch (error) {
    console.error("API Error fetching products:", error);
    throw new Error(String(error));
  }
}

export async function apiAddProduct(payload: ProductPayload): Promise<void> {
  console.log("API: Adding product...", payload);
  try {
    await invoke('add_product', { product: payload });
    console.log("API: Product added successfully.");
  } catch (error) {
    console.error("API Error adding product:", error);
    throw new Error(String(error));
  }
}
