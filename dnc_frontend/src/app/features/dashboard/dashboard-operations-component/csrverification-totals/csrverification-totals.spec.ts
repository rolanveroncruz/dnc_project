import { ComponentFixture, TestBed } from '@angular/core/testing';

import { CSRVerificationTotals } from './csrverification-totals';

describe('CSRVerificationTotals', () => {
  let component: CSRVerificationTotals;
  let fixture: ComponentFixture<CSRVerificationTotals>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [CSRVerificationTotals]
    })
    .compileComponents();

    fixture = TestBed.createComponent(CSRVerificationTotals);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
