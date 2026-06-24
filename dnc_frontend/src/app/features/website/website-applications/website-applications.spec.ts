import { ComponentFixture, TestBed } from '@angular/core/testing';

import { WebsiteApplications } from './website-applications';

describe('WebsiteApplications', () => {
  let component: WebsiteApplications;
  let fixture: ComponentFixture<WebsiteApplications>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [WebsiteApplications]
    })
    .compileComponents();

    fixture = TestBed.createComponent(WebsiteApplications);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
