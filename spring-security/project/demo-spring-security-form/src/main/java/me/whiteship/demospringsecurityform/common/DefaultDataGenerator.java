package me.whiteship.demospringsecurityform.common;

import me.whiteship.demospringsecurityform.account.Account;
import me.whiteship.demospringsecurityform.account.AccountService;
import me.whiteship.demospringsecurityform.book.Book;
import me.whiteship.demospringsecurityform.book.BookRepository;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.ApplicationArguments;
import org.springframework.boot.ApplicationRunner;

public class DefaultDataGenerator implements ApplicationRunner {

    @Autowired
    AccountService accountService;

    @Autowired
    BookRepository bookRepository;

    @Override
    public void run(ApplicationArguments args) throws Exception {
        Account joon = createUser("joon");
        Account seungho = createUser("seungho");

        Book spring = createBook("spring", joon);
        Book hibernate = createBook("hibernate", seungho);
    }

    private Book createBook(String title, Account joon) {
        Book book = new Book();
        book.setTitle(title);
        book.setAuthor(joon);
        return book;
    }

    private Account createUser(String username) {
        Account account = new Account();
        account.setUsername(username);
        account.setPassword("123");
        account.setRole("USER");
        return accountService.createNew(account);
    }
}
