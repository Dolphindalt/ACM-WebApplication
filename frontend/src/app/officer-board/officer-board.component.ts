import { Component, OnInit } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { ToastService } from '../toast.service';

@Component({
  selector: 'app-officer-board',
  templateUrl: './officer-board.component.html',
  styleUrls: ['./officer-board.component.sass']
})
export class OfficerBoardComponent implements OnInit {

  private officers: any;

  constructor(
    private http: HttpClient,
    private toastService: ToastService
  ) { }

  ngOnInit() {
    this.http.get("officers").subscribe(
      (res) => {
        this.officers = res;
      },
      error => {
        this.toastService.show("An error occured while fetching officers.", { classname: "bg-danger text-light" });
      }
    );
  }

}
