# GCC1919-Agenda

**GCC1919-Agenda** is a Rust-based command-line application for managing contacts and appointments. It provides a simple and efficient way to organize your contacts and schedule through options for adding, listing, updating, and deleting entries.

## Features

### Contacts
- Add a contact
- List contacts
- Update a contact
- Delete a contact

### Appointments
- Add an appointment
- List appointments
- Update an appointment
- Delete an appointment

## Data Structures

### Contato

```rust
struct Contato {
    nome: String,
    telefone: String,
    email: String,
    idade: u32,
    data_aniversario: String,
    categoria: Categoria
}
```

### Categoria

```rust
enum Categoria {
    Amigo,
    Familiar,
    Colega,
    Outro,
}
```

### Compromisso

```rust
struct Compromisso{
    titulo: String,
    data: String,
    hora: String,
    descricao: String,
    prioridade: Prioridade
}
```

### Prioridade
```rust
enum Prioridade {
    Alta,
    Media,
    Baixa,
}
```

## Usage

### Managing Contacts
Navigate to the contacts menu and select an option:
1. **Add a contact**: Add a new contact by providing the required information.
2. **List contacts**: Display a list of all contacts.
3. **Update a contact**: Modify details of an existing contact.
4. **Delete a contact**: Remove a contact from the list.
5. **Exit**: Exit the contacts menu.

### Managing Appointments
Navigate to the appointments menu and select an option:
1. **Add an appointment**: Add a new appointment by specifying the details.
2. **List appointments**: Display a list of all appointments.
3. **Update an appointment**: Modify details of an existing appointment.
4. **Delete an appointment**: Remove an appointment from the list.
5. **Exit**: Exit the appointments menu.

## Running the Application

Run the application using the following command:

```bash
cargo run
