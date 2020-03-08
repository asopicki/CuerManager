import { Component, OnInit, Input, ViewEncapsulation } from '@angular/core';

@Component({
  selector: 'app-issue-badge',
  templateUrl: './issue-badge.component.html',
  styleUrls: ['./issue-badge.component.scss'],
  encapsulation: ViewEncapsulation.None
})
export class IssueBadgeComponent implements OnInit {

  @Input() issueMessages: String[]
  @Input() key: String

  constructor() { }

  ngOnInit() {
  }

  issueCount(): number {
    return this.issueMessages[this.key.toString()] ? this.issueMessages[this.key.toString()].length : 0;
  }

  issues(): String {
    return this.issueMessages[this.key.toString()].join("  ---  ") || "";
  }

}
