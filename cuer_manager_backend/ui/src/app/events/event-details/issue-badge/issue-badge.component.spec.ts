import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { IssueBadgeComponent } from './issue-badge.component';

describe('IssueBadgeComponent', () => {
  let component: IssueBadgeComponent;
  let fixture: ComponentFixture<IssueBadgeComponent>;

  beforeEach(async(() => {
    TestBed.configureTestingModule({
      declarations: [ IssueBadgeComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(IssueBadgeComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
