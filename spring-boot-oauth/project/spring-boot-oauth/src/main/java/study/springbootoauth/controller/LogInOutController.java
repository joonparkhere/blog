package study.springbootoauth.controller;

import org.springframework.stereotype.Controller;
import org.springframework.ui.Model;
import org.springframework.web.bind.annotation.GetMapping;
import study.springbootoauth.annotation.LoginUser;
import study.springbootoauth.dto.SessionUser;

@Controller
public class LogInOutController {

    @GetMapping("/login")
    public String login() {
        return "login";
    }

    @GetMapping("/logout")
    public String logout() {
        return "logout";
    }

}
