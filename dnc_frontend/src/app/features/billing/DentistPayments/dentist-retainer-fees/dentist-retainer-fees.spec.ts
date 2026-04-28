import { ComponentFixture, TestBed } from '@angular/core/testing';

import { DentistRetainerFees } from './dentist-retainer-fees';

describe('DentistRetainerFees', () => {
  let component: DentistRetainerFees;
  let fixture: ComponentFixture<DentistRetainerFees>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [DentistRetainerFees]
    })
    .compileComponents();

    fixture = TestBed.createComponent(DentistRetainerFees);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
