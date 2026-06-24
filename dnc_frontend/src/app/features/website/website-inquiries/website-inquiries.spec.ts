import { ComponentFixture, TestBed } from '@angular/core/testing';

import { WebsiteInquiries } from './website-inquiries';

describe('WebsiteInquiries', () => {
  let component: WebsiteInquiries;
  let fixture: ComponentFixture<WebsiteInquiries>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [WebsiteInquiries]
    })
    .compileComponents();

    fixture = TestBed.createComponent(WebsiteInquiries);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
