import { ComponentFixture, TestBed } from '@angular/core/testing';

import { SetupClinics } from './setup-clinics';

describe('SetupClinics', () => {
  let component: SetupClinics;
  let fixture: ComponentFixture<SetupClinics>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [SetupClinics]
    })
    .compileComponents();

    fixture = TestBed.createComponent(SetupClinics);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
