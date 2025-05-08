# 🔄 Migration Plan: SQLite to Supabase for SalesTracking

This document outlines a future migration strategy from the current local SQLite backend (used via Tauri + Rust) to a cloud-based Supabase PostgreSQL database.

---

## ✅ Phase 1: Current Setup (SQLite)
- Local database file: `sales_tracker.db`
- Tables: `salespeople`, `products`, etc.
- Managed by Rust using `rusqlite`
- Schema defined in `schema.rs`
- Accessed via Tauri commands like `get_salespeople`, `add_product`

---

## 📦 Phase 2: Prepare Supabase
1. **Create Supabase Project**
   - Go to https://supabase.com and sign in
   - Create a new project with a database password

2. **Replicate Schema**
   - Use SQL editor to manually create the tables:
     ```sql
     CREATE TABLE salespeople (
       id SERIAL PRIMARY KEY,
       name TEXT NOT NULL,
       employee_id TEXT,
       email TEXT,
       is_active BOOLEAN DEFAULT TRUE
     );

     CREATE TABLE products (
       id SERIAL PRIMARY KEY,
       sku TEXT UNIQUE NOT NULL,
       name TEXT NOT NULL,
       brand TEXT NOT NULL,
       category TEXT,
       cost_price REAL NOT NULL,
       unit_price REAL NOT NULL,
       description TEXT
     );
     ```

3. **Enable API Access**
   - Generate service role key and anon/public API key
   - Whitelist necessary domains for CORS

---

## 📤 Phase 3: Migrate Data
1. **Export from SQLite**
   - Use SQLite browser or script to dump tables to CSV

2. **Import to Supabase**
   - Use Supabase table editor or SQL bulk `COPY` command

---

## 🔁 Phase 4: Replace Backend Logic

### Option 1: Move logic to TypeScript (frontend)
- Use Supabase JS client: `@supabase/supabase-js`
- Replace `invoke("get_salespeople")` with direct API calls

### Option 2: Keep Rust, use HTTP client
- Make HTTP requests to Supabase REST endpoints or RPC
- Auth handled via header token

---

## ✅ Benefits of Supabase
- Central cloud access (shared between devices)
- Built-in auth, permissions, backups
- Scalable PostgreSQL backend

---

## 🔚 Final Step
- Remove SQLite dependency
- Remove `src/database/` folder
- Update API layer to point to Supabase instead

---

### 📝 Notes
- Migration is optional until launch
- Offline/local-only mode is better during development
