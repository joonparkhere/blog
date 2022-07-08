package com.example.demowebmvc;

import org.springframework.http.HttpHeaders;
import org.springframework.http.MediaType;
import org.springframework.stereotype.Controller;
import org.springframework.ui.Model;
import org.springframework.validation.BindingResult;
import org.springframework.validation.annotation.Validated;
import org.springframework.web.bind.annotation.*;
import org.springframework.web.bind.support.SessionStatus;
import org.springframework.web.servlet.mvc.support.RedirectAttributes;

import javax.validation.Valid;
import java.util.ArrayList;
import java.util.List;
import java.util.Map;

@Controller
@RequestMapping("/sample")
@SessionAttributes("event")
public class SampleController {

    @ModelAttribute("categories")
    public List<String> categories(Model model) {
        return List.of("study", "seminar", "hobby");
    }
//    @ModelAttribute
//    public void categories(Model model) {
//        model.addAttribute("categories", List.of("study", "seminar", "hobby"));
//    }

//    @RequestMapping(value = "/hello", method = {RequestMethod.GET, RequestMethod.PUT)
//    @ResponseBody   // return을 string으로 반환 (원래는 view로 반환)
//    public String hello() {
//        return "hello";
//    }

    @GetMapping("")
    @ResponseBody
    public String sample() {
        return "sample";
    }

    @GetMapping(
            value = "/hello",
            consumes = MediaType.APPLICATION_JSON_VALUE,
            produces = MediaType.TEXT_PLAIN_VALUE,
            headers = HttpHeaders.FROM + "=" + "111",
            params = "name=spring")
    @ResponseBody
    public String hello() {
        return "hello";
    }


    @GetMapping("/hello/{name:[a-z]+}")
    @ResponseBody
    public String helloName(@PathVariable String name) {
        return "hello " + name;
    }

    @GetMapping("/events/form")
    public String eventsForm(Model model) {
        Event newEvent = new Event();
        newEvent.setName("spring");
        newEvent.setLimit(3);
        model.addAttribute("event", newEvent);
        return "/events/form";
    }

    @PostMapping("/events")
    public String createEvents(@Validated @ModelAttribute Event event,
                               BindingResult bindingResult,
                               SessionStatus sessionStatus) {
        if(bindingResult.hasErrors()) {
            return "/events/form";
        }
        sessionStatus.setComplete();
        System.out.println("### eventName: " + event.getName());
        return "redirect:/sample/events/list";
    }
//    @ResponseBody
//    public Event getEvent(@RequestParam String name,
//                          @RequestParam Integer limit) {
//        Event event = new Event();
//        event.setName(name);
//        event.setLimit(limit);
//        return event;
//    }
//    public Event getEventName(@RequestParam Map<String, String> params) {
//        Event event = new Event();
//        event.setName(params.get("name"));
//        return event;
//    }

    @GetMapping("/events/form/name")
    public String eventsFormName(Model model) {
        model.addAttribute("event", new Event());
        return "/events/form-name";
    }

    @PostMapping("/events/form/name")
    public String eventsFormName(@Validated @ModelAttribute Event event,
                               BindingResult bindingResult,
                               SessionStatus sessionStatus) {
        if(bindingResult.hasErrors()) {
            return "/events/form-name";
        }
        return "redirect:/sample/events/form/limit";
    }

    @GetMapping("/events/form/limit")
    public String eventsFormLimit(@ModelAttribute Event event, Model model) {
        model.addAttribute("event", event);
        return "/events/form-limit";
    }

    @PostMapping("/events/form/limit")
    public String eventsFormLimit(@Validated @ModelAttribute Event event,
                                  BindingResult bindingResult,
                                  SessionStatus sessionStatus,
                                  RedirectAttributes attributes) {
        if(bindingResult.hasErrors()) {
            return "/events/form-limit";
        }
        sessionStatus.setComplete();
//        attributes.addAttribute("name", event.getName());
//        attributes.addAttribute("limit", event.getLimit());
        attributes.addFlashAttribute("newEvent", event);
        return "redirect:/sample/events/list";
    }

    @GetMapping("/events/list")
    public String getEvents(Model model) {

        Event defaultEvent = new Event();
        defaultEvent.setName("spring");
        defaultEvent.setLimit(1);

        Event newEvent = (Event) model.asMap().get("newEvent");

        List<Event> eventList = new ArrayList<>();
        eventList.add(defaultEvent);
        eventList.add(newEvent);

        model.addAttribute(eventList);

        return "/events/list";
    }
//    public String getEvents(@ModelAttribute("newEvent") Event event,
//                            Model model) {
//
//        Event newEvent = new Event();
//        newEvent.setName("spring");
//        newEvent.setLimit(1);
//
//        List<Event> eventList = new ArrayList<>();
//        eventList.add(event);
//        eventList.add(newEvent);
//
//        model.addAttribute(eventList);
//
//        return "/events/list";
//    }
//    public String getEvents(@RequestParam String name,
//                            @RequestParam Integer limit,
//                            Model model) {
//        Event myEvent = new Event();
//        myEvent.setName(name);
//        myEvent.setLimit(limit);
//
//        Event newEvent = new Event();
//        newEvent.setName("spring");
//        newEvent.setLimit(1);
//
//        List<Event> eventList = new ArrayList<>();
//        eventList.add(myEvent);
//        eventList.add(newEvent);
//
//        model.addAttribute(eventList);
//
//        return "/events/list";
//    }

    // /sample/events/1;name=spring
    @GetMapping("/events/{id}")
    @ResponseBody
    public Event getEvent(@PathVariable Integer id, @MatrixVariable String name) {
        Event event = new Event();
        event.setId(id);
        event.setName(name);
        return event;
    }
}
