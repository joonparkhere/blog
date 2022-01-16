package study.springbootoauth.controller;

import lombok.RequiredArgsConstructor;
import org.springframework.security.core.context.SecurityContext;
import org.springframework.security.core.context.SecurityContextHolder;
import org.springframework.stereotype.Controller;
import org.springframework.ui.Model;
import org.springframework.web.bind.annotation.GetMapping;
import study.springbootoauth.annotation.LoginUser;
import study.springbootoauth.dto.SessionUser;

import javax.servlet.http.HttpSession;
import java.security.Principal;

@Controller
@RequiredArgsConstructor
public class SampleController {

    private final HttpSession httpSession;

//    @GetMapping("/")
//    public String index(Model model) {
//        SessionUser user = (SessionUser) httpSession.getAttribute("user");
//
//        if (user != null) {
//            model.addAttribute("userName", user.getName());
//        }
//
//        return "index";
//    }

    @GetMapping("/")
    public String index(Model model, @LoginUser SessionUser user) {
        SecurityContext securityContext = SecurityContextHolder.getContext();
        HttpSession httpSession = this.httpSession;
        if(user != null) {
            model.addAttribute("name", user.getName());
        }
        return "index";
    }

    @GetMapping("/dashboard")
    public String dashboard(Model model, @LoginUser SessionUser user) {
        if(user != null) {
            model.addAttribute("picture", user.getPicture());
            model.addAttribute("message",
                    "Hello, " + user.getName() +
                            " (" + user.getEmail() + ")"
            );
            model.addAttribute("profileImage", user.getPicture());
        }
        return "dashboard";
    }

}
