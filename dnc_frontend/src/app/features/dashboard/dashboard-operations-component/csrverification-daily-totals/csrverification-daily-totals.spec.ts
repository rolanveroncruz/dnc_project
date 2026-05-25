import { ComponentFixture, TestBed } from '@angular/core/testing';

import { CSRVerificationDailyTotals } from './csrverification-daily-totals';

describe('CSRVerificationDailyTotals', () => {
  let component: CSRVerificationDailyTotals;
  let fixture: ComponentFixture<CSRVerificationDailyTotals>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [CSRVerificationDailyTotals]
    })
    .compileComponents();

    fixture = TestBed.createComponent(CSRVerificationDailyTotals);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
