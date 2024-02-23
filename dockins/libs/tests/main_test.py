import sqlite3
from tkinter import Tk, Label, Entry, Button, messagebox
from datetime import date

class Book:
    def __init__(self, title, author, year):
        self.title = title
        self.author = author
        self.year = year
        
    def displayInfo(self):
        print('Title:', self.title)
        print('Author:', self.author)
        print('Year Published:', self.year)

class LibraryDatabase:
    def __init__(self):
        self.conn = sqlite3.connect('library.db')
        self.cursor = self.conn.cursor()
        self._createTable()

    def _createTable(self):
        query = 'CREATE TABLE IF NOT EXISTS Books (' \
                'id INTEGER PRIMARY KEY AUTOINCREMENT,' \
                'title TEXT UNIQUE NOT NULL,' \
                'author TEXT NOT NULL,' \
                'year INT)'
        self.cursor.execute(query)
        self.conn.commit()

    def insertBook(self, book):
        values = (book.title, book.author, book.year)
        query = 'INSERT INTO Books VALUES (NULL, ?, ?, ?)'
        self.cursor.execute(query, values)
        self.conn.commit()

    def searchBooksByTitle(self, title):
        query = 'SELECT id, title, author, year FROM Books WHERE title LIKE ?'
        params = '%{}%'.format(title)
        self.cursor.execute(query, (params,))
        rows = self.cursor.fetchall()
        return [Book(*row) for row in rows]

    def deleteBookById(self, bookId):
        query = 'DELETE FROM Books WHERE id=?'
        self.cursor.execute(query, (bookId,))
        self.conn.commit()

def showGUI():
    window = Tk()
    window.geometry('600x700+300+200')
    window.title('Library Catalog Manager')

    label_title = Label(window, text='Welcome to the Library Catalog Manager!', font='Helvetica 28 bold').pack()

    def addNewBook():
        title = entry_title.get().strip()
        author = entry_author.get().strip()
        year = entry_year.get().strip()

        if not all([title, author, year]):
            messagebox.showerror('Invalid Input', 'Please enter valid information for Title, Author, and Year.')
            return

        book = Book(title, author, int(year))
        db.insertBook(book)
        messagebox.showinfo('Successful', f'{book.displayInfo()} was successfully added to the library catalog.')

    def searchForBook():
        title = entry_search.get().lower().strip()
        if not title:
            messagebox.showerror('Empty Search Term', 'Please enter a non-empty string to search for.')
            return

        found_books = db.searchBooksByTitle(title)
        if len(found_books) == 0:
            messagebox.showwarning('No Results Found', 'There were no matching titles found in the library catalog.')
            return

        for i, book in enumerate(found_books):
            print('\nMatch #{}: {} ({})'.format(i+1, book.title, book.author), end="\n")

    frame_new_book = Frame(window).pack()
    Label(frame_new_book, text='Enter New Book Information:').grid(columnspan=2)
    entry_title = Entry(frame_new_book).grid(row=1, column=0)
    entry_author = Entry(frame_new_book).grid(row=2, column=0)
    entry_year = Entry(frame_new_book).grid(row=3, column=0)
    button_submit = Button(frame_new_book, text='Submit', command=addNewBook).grid(row=4, column=0)

    frame_search = Frame(window).pack()
    Label(frame_search, text='Search Existing Titles:').grid(columnspan=2)
    entry_search = Entry(frame_search).grid(row=1, column=0)
    button_find = Button(frame_search, text='Find', command=searchForBook).grid(row=2, column=0)

    window.mainloop()

if __name__ == '__main__':
    db = LibraryDatabase()
    showGUI()