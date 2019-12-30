import { Component, OnInit, Input, Inject } from '@angular/core';
import { User } from '../models/user';

@Component({
  selector: 'app-user',
  templateUrl: './user.component.html',
  styleUrls: ['./user.component.sass']
})
export class UserComponent implements OnInit {

  @Input() user: User;
  private backend_url: String;

  constructor(
    @Inject('BASE_API_URL') private baseUrl: string
  ) {
    this.backend_url = baseUrl;
  }

  ngOnInit() {
  }

}
