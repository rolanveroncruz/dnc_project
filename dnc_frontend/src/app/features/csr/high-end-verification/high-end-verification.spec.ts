import { ComponentFixture, TestBed } from '@angular/core/testing';

import { HighEndVerification } from './high-end-verification';

describe('HighEndVerification', () => {
  let component: HighEndVerification;
  let fixture: ComponentFixture<HighEndVerification>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [HighEndVerification]
    })
    .compileComponents();

    fixture = TestBed.createComponent(HighEndVerification);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
