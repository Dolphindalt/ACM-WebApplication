import { Component, OnInit } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { ToastService } from '../toast.service';

@Component({
  selector: 'app-event-board',
  templateUrl: './event-board.component.html',
  styleUrls: ['./event-board.component.sass']
})
export class EventBoardComponent implements OnInit {

  private future_events: any;
  private past_events: any;

  constructor(
    private http: HttpClient,
    public toastService: ToastService
  ) {

  }

  ngOnInit() {
    this.http.get("event").subscribe(
      (res) => {
        this.future_events = res[0];
        this.past_events = res[1];
      },
      error => {
        this.toastService.show("An error occured while fetching events.", { classname: "bg-danger text-light" });
      }
    );
  }
}
